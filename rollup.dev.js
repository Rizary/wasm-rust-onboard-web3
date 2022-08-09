import commonjs from '@rollup/plugin-commonjs';
import json from "@rollup/plugin-json";
import nodeResolve from "@rollup/plugin-node-resolve";
import rust from "@wasm-tool/rollup-plugin-rust";
import injectProcessEnv from "rollup-plugin-inject-process-env";
import livereload from "rollup-plugin-livereload";
// import external from 'rollup-plugin-peer-deps-external';
import nodePolyfills from 'rollup-plugin-polyfill-node';
import serve from "rollup-plugin-serve";
import { getEnv } from "./rollup.common.js";

export default {
    input: {
        index: "Cargo.toml"
    },
    output: {
        dir: "devhtml/js",
        format: "iife",
        sourcemap: true,
        chunkFileNames: "[name].js",
        assetFileNames: "assets/[name][extname]",
        inlineDynamicImports: true,
        // external: [ '@web3-onboard/core', "@web3-onboard/walletconnect" ],
        // globals: {
        //     'Onboard': '@web3-onboard/core',
        //     'walletConnectModule': '@web3-onboard/walletconnect',
        // }
    },
    plugins: [
        rust({
            serverPath: "/js/",
            debug: true,
            verbose: true,
            watchPatterns: ["./src/**", "./js/**"],
            cargoArgs: ["--features", "develop"],
            watch: true,
        }),

        // external({
        //     includeDependencies: true,
        // }),
        nodeResolve({
            preferBuiltins: 'false',
        }),

        commonjs(),
        
        nodePolyfills({ 
            include: [
                '*.js',
               'node_modules/**/*.js',
                new RegExp('node_modules/.vite/.*js')
              ],
              // ↓ Not sure if this line is necessary, seems to work without it
              exclude: ['node_modules/polyfill-nodeglobal.js']
         }),

        json(),

        serve({
            contentBase: 'devhtml',
            open: true,
            verbose: true,
            debug: false,
            port: 8189,
            headers: {
                'Access-Control-Allow-Origin': '*',
                "Content-Type": "application/wasm",
            },
            historyApiFallback: true,
        }),

        livereload({
            watch: 'devhtml/js',
            verbose: true
        }),
        
        injectProcessEnv(getEnv()),
    ],
    watch: {
        clearScreen: false
    }
};