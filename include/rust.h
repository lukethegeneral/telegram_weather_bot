typedef struct
{
	double longitude;
	double latitude;
} GPScoordinates;

typedef struct 
{
    const char *temp_unit;
    float temp_value;
	bool error_flg;
	const char *error_msg;
} CurrentWeather;

extern CurrentWeather *get_current_temperature_c(GPScoordinates gps);
extern void get_current_temperature_c_free(CurrentWeather *ptr);