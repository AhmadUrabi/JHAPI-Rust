INSERT INTO ODBC_JHC.API_LOGS (
    USERNAME,
    ROUTE,
    PARAMETERS,
    TIMESTAMP,
    RESULT,
    TOKEN_USED,
    IP_ADDRESS,
    METHOD
) VALUES (
    :USERNAME,
    :ROUTE,
    :PARAMETERS,
    :TIMESTAMP,
    :RESULT,
    :TOKEN_USED,
    :IP_ADDRESS,
    :METHOD
)