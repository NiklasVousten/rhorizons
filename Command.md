# Command

Commands/Query are implemented according to [version 1.2](https://ssd-api.jpl.nasa.gov/doc/horizons.html).  
Supported Parameters are listed below.

## Currently implemented Query Parameters and Ephemeris Types

- [Ephemeris Types](#ephemeris-types)
- [Common Parameters](#common-parameters)
- [Ephemeris-Specific Parameters](#ephemeris-specific-parameters)
- [SPK File Parameters](#spk-file-parameters)
- [Close-Approach Table Parameters](#close-approach-table-parameters)

### Ephemeris Types

| Type      | Implemented           |
|-----------|-----------------------|
| OBSERVER  | :white_check_mark:    |
| ELEMENTS  | :white_check_mark:    |
| VECTORS   | :white_check_mark:    |
| APPROACH  | :x:                   |
| SPK       | :x:                   |

### Common Parameters

| Paramter      | Implemented           |
|---------------|-----------------------|
| format        | :white_check_mark:    |
| COMMAND       | :white_check_mark:    |
| OBJ_DATA      | :white_check_mark:    |
| MAKE_EPHEM    | :white_check_mark:    |
| EPHEM_TYPE    | :white_check_mark:    |
| EMAIL_ADDR    | :white_check_mark:    |

### Ephemeris-Specific Parameters

| Parameter             | Observer              | Vectors               | Elements              |
|-----------------------|-----------------------|-----------------------|-----------------------|
| CENTER                | :white_check_mark:    | :white_check_mark:    | :white_check_mark:    |
| REF_PLANE             |                       | :x:                   | :x:                   |
| COORD_TYPE            | :x:                   | :x:                   | :x:                   |
| SITE_COORD            | :x:                   | :x:                   | :x:                   |
| START_TIME            | :x:                   | :x:                   | :x:                   |
| STOP_TIME             | :x:                   | :x:                   | :x:                   |
| STEP_SIZE             | :x:                   | :x:                   | :x:                   |
| TLIST                 | :x:                   | :x:                   | :x:                   |
| TLIST_TYPE            | :x:                   | :x:                   | :x:                   |
| QUANTITIES            | :x:                   |                       |                       |
| REF_SYSTEM            | :x:                   | :x:                   | :x:                   |
| OUT_UNITS             |                       | :x:                   | :x:                   |
| VEC_TABLE             |                       | :x:                   |                       |
| VEC_CORR              |                       | :x:                   |                       |
| CAL_FORMAT            | :x:                   |                       |                       |
| CAL_TYPE              | :x:                   | :x:                   | :x:                   |
| ANG_FORMAT            | :x:                   |                       |                       |
| APPARENT              | :x:                   |                       |                       |
| TIME_DIGITS           | :x:                   | :x:                   | :x:                   |
| TIME_ZONE             | :x:                   |                       |                       |
| RANGE_UNITS           | :x:                   |                       |                       |
| SUPPRESS_RANGE_RATE   | :x:                   |                       |                       |
| ELEV_CUT              | :x:                   |                       |                       |
| SKIP_DAYLT            | :x:                   |                       |                       |
| SOLAR_ELONG           | :x:                   |                       |                       |
| AIRMASS               | :x:                   |                       |                       |
| LHA_CUTOFF            | :x:                   |                       |                       |
| ANG_RATE_CUTOFF       | :x:                   |                       |                       |
| EXTRA_PREC            | :x:                   |                       |                       |
| CSV_FORMAT            | :x:                   | :x:                   | :x:                   |
| VEC_LABELS            |                       | :x:                   |                       |
| VEC_DELTA_T           |                       | :x:                   |                       |
| ELM_LABELS            |                       |                       | :x:                   |
| TP_TYPE               |                       |                       | :x:                   |
| R_T_S_ONLY            | :x:                   |                       |                       |

### SPK File Parameters

| Paramter      | Implemented   |
|---------------|---------------|
| START_TIME    | :x:           |
| STOP_TIME     | :x:           |

### Close-Approach Table Parameters

| Paramter      | Implemented   |
|---------------|---------------|
| CA_TABLE_TYPE | :x:           |
| TCA3SG_LIMIT  | :x:           |
| CALIM_SB      | :x:           |
| CALIM_PL      | :x:           |
