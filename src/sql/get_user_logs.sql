SELECT * FROM ODBC_JHC.API_LOGS WHERE USERNAME = :username ORDER BY TIMESTAMP DESC FETCH NEXT :limit ROWS ONLY