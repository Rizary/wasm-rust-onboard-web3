require("dotenv").config({ path: __dirname + "/./../../.env" });

export function getEnv() {
    return {
        WALLETCONNECT_ID: process.env.WALLETCONNECT_ID,
        POLYGON_TEST_URL: process.env.POLYGON_TEST_URL,
        POLYGON_MATICVIGIL_API_KEY: process.env.POLYGON_MATICVIGIL_API_KEY,
        POLYGON_MAIN_URL: process.env.POLYGON_MAIN_URL,
    };
}