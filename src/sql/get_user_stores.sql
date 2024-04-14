SELECT
    LPAD(S.STORE_ID, 2, '0') STORE_ID,
    S.STORE_DESC,
    S.STORE_DESC_S
FROM
    ODBC_JHC.JHC_STORES         S
WHERE
    EXISTS (
        SELECT
            1
        FROM
            ODBC_JHC.USER_STORES_JHC    USA
            JOIN ODBC_JHC.AUTHENTICATION_JHC U
            ON USA.USERNAME = U.USERNAME
        WHERE
            (U.USERNAME = :USER_ID
            AND USA.ALL_STORES_ACCESS = 1)
            OR (U.USERNAME = :USER_ID
            AND USA.STORE_ID = S.STORE_ID)
    )