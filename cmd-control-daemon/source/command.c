#include "cmd-control-daemon/daemon.h"
#include <csp/csp.h>
#include <dlfcn.h>
#include <stdlib.h>
#include <stdio.h>
#include <time.h>
#include <unistd.h>
#include "tinycbor/cbor.h"

#define CBOR_BUF_SIZE YOTTA_CFG_CSP_MTU
#define SO_PATH_LENGTH 75


bool get_command(csp_socket_t* sock, char * command) {
    csp_conn_t *conn;
    csp_packet_t *packet;

    while (1) {
        conn = csp_accept(sock, 1000);
        if (conn) {
            packet = csp_read(conn, 0);
            if (packet)
            {
                if (!parse_command_cbor(packet, command)){
                    fprintf(stderr, "There was an error parsing the command packet\n");
                    return false;
                }
            }
            csp_buffer_free(packet);
            csp_close(conn);
            return true;
        }
    }
}


bool parse_command_cbor(csp_packet_t * packet, char * command) {
    CborParser parser;
    CborValue map, element;
    size_t len;

    CborError err = cbor_parser_init((uint8_t*) packet->data, packet->length, 0, &parser, &map);
    if (err)
        return false;

    err = cbor_value_map_find_value(&map, "ARGS", &element);
    if (err || cbor_value_copy_text_string(&element, command, &len, NULL))
        return false;
    return true;
}


bool load_command(cnc_command_wrapper * wrapper, void ** handle, lib_function * func) {
    int return_code;
    char so_path[SO_PATH_LENGTH];
    char * home_dir = "/home/vagrant/lib%s.so";

    // so_len - the format specifier length (-2) + the null character (+1) leading to the -1
    int so_len = strlen(home_dir) + strlen(wrapper->command_packet->cmd_name) - 1;
    snprintf(so_path, so_len, home_dir, wrapper->command_packet->cmd_name);

    *handle = dlopen(so_path, RTLD_NOW | RTLD_GLOBAL);

    if (*handle == NULL)
    {
        wrapper->err = true;
        sprintf(wrapper->output, "Unable to open lib: %s\n", dlerror());
        return false;
    }

    switch (wrapper->command_packet->action){
        case execute:
            printf("Running Command Execute\n");
            *func = dlsym(*handle, "execute");
            break;
        case status:
            printf("Running Command status\n");
            func = dlsym(*handle, "status");
            break;
        case output:
            printf("Running Command output\n");
            func = dlsym(*handle, "output");
            break;
        case help:
            printf("Running Command help\n");
            func = dlsym(*handle, "help");
            break;
        default:
            sprintf(wrapper->output, "Unable to open lib: %s\n", dlerror());
            return false;
    }

    if (func == NULL) {
        sprintf(wrapper->output, "The requested Symbol doesn't exist\n");
        return false;
    }
    return true;
}


bool run_command(cnc_command_wrapper * wrapper, void ** handle, lib_function func) {
    //Redirect stdout to the response output field.
    //TODO: Redirect or figure out what to do with STDERR

    int original_stdout;
    fflush(stdout);
    original_stdout = dup(STDOUT_FILENO);
    freopen("/dev/null", "a", stdout);
    setbuf(stdout, wrapper->response_packet->output);

    //Measure the clock before and after the function has run
    clock_t start_time = clock();
    wrapper->response_packet->return_code = func(wrapper->command_packet->arg_count, wrapper->command_packet->args);
    clock_t finish_time = clock();

    //Redirect stdout back to the terminal.
    freopen("/dev/null", "a", stdout);
    dup2(original_stdout, STDOUT_FILENO); //restore the previous state of stdout
    setbuf(stdout, NULL);

    //Calculate the runtime
    wrapper->response_packet->execution_time = (double)(finish_time - start_time) / (CLOCKS_PER_SEC/1000); //execution time in milliseconds
    printf("Return code: %i exection time %f\n", wrapper->response_packet->return_code, wrapper->response_packet->execution_time);

    //Unload the library
    func = NULL;
    dlclose(*handle);

    return true;
}


bool process_and_run_command(cnc_command_wrapper * wrapper) {
    lib_function func = NULL;
    void * handle;

    /*if (wrapper == NULL)*/
        /*wrapper->err = true;*/
        /*//TODO:Generate a usefule error message for this case*/
        /*return false;*/

    if (!load_command(wrapper, &handle, &func)) {
        printf("Failed to load command\n");
        wrapper->err = true;
        send_result(wrapper);
        return false;
    }
    if (!run_command(wrapper, &handle, func)) {
        printf("Failed to run command\n");
        wrapper->err = true;
        send_result(wrapper);
        return false;
    }

    //Running the command succeeded
    printf("Command succeeded - Sending Result\n");
    return send_result(wrapper);
}


bool send_result(cnc_command_wrapper * wrapper)
{
    printf("Sending result\n");
    if (wrapper->err) { //Thinking of changing the err flag to a state enum or similar multi-state member type
        start_encode_response(RESPONSE_TYPE_PROCESSING_ERROR, wrapper);
    } else {
        start_encode_response(RESPONSE_TYPE_COMMAND_RESULT, wrapper);
    }
}


bool start_encode_response(int message_type, cnc_command_wrapper * wrapper) {
    CborEncoder encoder, container;
    CborError err;
    uint8_t data[MTU] = {0};

    cbor_encoder_init(&encoder, data, MTU, 0);
    err = cbor_encoder_create_map(&encoder, &container, 4); //TODO: Dynamically assign map size
    if (err)
        return false;

    err = cbor_encode_text_stringz(&container, "MSG_TYPE");
    if (cbor_encode_int(&container, message_type))
        return false;

    switch (message_type)
    {
        case RESPONSE_TYPE_COMMAND_RESULT:
            return encode_response(data, wrapper, &encoder, &container);
        case RESPONSE_TYPE_PROCESSING_ERROR:
            return encode_processing_error(data, wrapper, &encoder, &container);
    }
}


bool encode_response(uint8_t * data, cnc_command_wrapper * wrapper, CborEncoder * encoder, CborEncoder * container) {
    CborError err;
    err = cbor_encode_text_stringz(container, "RETURN_CODE");
    if (err || cbor_encode_simple_value(container, wrapper->response_packet->return_code))
        return false;

    err = cbor_encode_text_stringz(container, "EXEC_TIME");
    if (err || cbor_encode_double(container, wrapper->response_packet->execution_time))
        return false;

    err = cbor_encode_text_stringz(container, "OUTPUT");
    if (err || cbor_encode_text_stringz(container, wrapper->response_packet->output))
        return false;

    return finish_encode_response_and_send(data, encoder, container);
}


bool encode_processing_error(uint8_t * data, cnc_command_wrapper * result, CborEncoder * encoder, CborEncoder * container) {
    //TODO: finish encoding stuff
    CborError err;
    printf("Encoding Processing Error\n");
    err = cbor_encode_text_stringz(container, "ERROR_MSG");
    if (err || cbor_encode_text_stringz(container, result->output))
        return false;

    return finish_encode_response_and_send(data, encoder, container);
}


bool finish_encode_response_and_send(uint8_t * data, CborEncoder *encoder, CborEncoder * container) {
    printf("sending response packet\n");
    cbor_encoder_close_container(encoder, container);
    size_t data_len = cbor_encoder_get_buffer_size(encoder, data);
    return send_response(data, data_len);
}

