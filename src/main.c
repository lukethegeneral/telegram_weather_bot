#include <stdio.h>
#include <stdlib.h>
#include <pthread.h>
#include <string.h>
#include <unistd.h>
#include <telebot.h>
#include "rust.h"

#define SIZE_OF_ARRAY(array) (sizeof(array) / sizeof(array[0]))

int main(int argc, char *argv[])
{
    printf("Welcome to Echobot\n");

    FILE *fp = fopen(".token", "r");
    if (fp == NULL)
    {
        printf("Failed to open .token file\n");
        return -1;
    }

    char token[1024];
    if (fscanf(fp, "%s", token) == 0)
    {
        printf("Failed to read token\n");
        fclose(fp);
        return -1;
    }
    printf("Token: %s\n", token);
    fclose(fp);

    telebot_handler_t handle;
    if (telebot_create(&handle, token) != TELEBOT_ERROR_NONE)
    {
        printf("Telebot create failed\n");
        return -1;
    }

    telebot_user_t me;
    if (telebot_get_me(handle, &me) != TELEBOT_ERROR_NONE)
    {
        printf("Failed to get bot information\n");
        telebot_destroy(handle);
        return -1;
    }

    printf("ID: %d\n", me.id);
    printf("First Name: %s\n", me.first_name);
    printf("User Name: %s\n", me.username);

    telebot_put_me(&me);

    int index, count, offset = -1;
    telebot_error_e ret;
    telebot_message_t message;
    telebot_update_type_e update_types[] = {TELEBOT_UPDATE_TYPE_MESSAGE};

    while (1)
    {
        telebot_update_t *updates;
        ret = telebot_get_updates(handle, offset, 20, 0, update_types, 0, &updates, &count);
        if (ret != TELEBOT_ERROR_NONE)
            continue;
        printf("Number of updates: %d\n", count);
        for (index = 0; index < count; index++)
        {
            message = updates[index].message;
            if (message.text)
            {
                printf("%s: %s \n", message.from->first_name, message.text);
                if (strstr(message.text, "/weather"))
                {
                    GPScoordinates gps = {0,0};

                    if (sscanf(message.text, "/weather %lf %lf", &gps.latitude, &gps.longitude) == 2) {
                        CurrentWeather *cw = get_current_temperature_c(gps);
                        printf("temp: %.2f%s\n", cw->temp_value, cw->temp_unit);
                        
                        char str[4096];
                        if(!cw->error_flg) 
                            snprintf(str, SIZE_OF_ARRAY(str), "Current temperature for location (%.2f,%.2f) is %.2f%s\n",
                                gps.latitude, gps.longitude, cw->temp_value, cw->temp_unit);
                        else
                            snprintf(str, SIZE_OF_ARRAY(str), "[error]:\n%s\n", cw->error_msg);

                        ret = telebot_send_message(handle, message.chat->id, str, "HTML", false, false, updates[index].message.message_id, "");
                        if (ret != TELEBOT_ERROR_NONE)
                        {
                            printf("Failed to send message: %d \n", ret);
                        }

                        //free string memory in rust
                        get_current_temperature_c_free(cw);
                    }
                    else
                    {
                        char *str = "Wrong arguments. Correct format:\n/weather latitude longitude\n";
                        printf("%s\n", str);
                        ret = telebot_send_message(handle, message.chat->id, str, "HTML", false, false, updates[index].message.message_id, "");
                        if (ret != TELEBOT_ERROR_NONE)
                        {
                            printf("Failed to send message: %d \n", ret);
                        }
                    }
                }
                if (ret != TELEBOT_ERROR_NONE)
                {
                    printf("Failed to send message: %d \n", ret);
                }
            }
            offset = updates[index].update_id + 1;
        }
        telebot_put_updates(updates, count);

        sleep(1);
    }

    telebot_destroy(handle);

    return 0;
}

