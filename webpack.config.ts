import WasmPackPlugin from "@wasm-tool/wasm-pack-plugin";
import CopyPlugin from "copy-webpack-plugin";
import path from 'path';

const BASE_PATH = path.join(process.cwd(), 'portfolio');

const distPath = path.join(BASE_PATH, 'dist');

export const mode = "production";
export const experiments = {
    asyncWebAssembly: true,
};

// export const module = {
//     rules: [{
//         test: /\.wasm$/,
//         type: "webassembly/async"
//     }]
// };

export const entry = {
    index: path.join(BASE_PATH, 'js', 'index.js')
};

// export const output = {
//     hashFunction: "xxhash64",
//     path: distPath,
//     filename: "[name].js"
// };
export const devServer = {
    static: [distPath],
};

// export const performance = { hints: false };

export const plugins = [
    new CopyPlugin({
        patterns: [
            path.join(BASE_PATH, 'static')
        ]
    }),

    new WasmPackPlugin({
        crateDirectory: BASE_PATH,
    })
];
