// const postcssAnimations = require("postcss-animations")({
//     data: [
//         require("postcss-animation.css-data"),
//         require("postcss-magic.css-data"),
//         require("postcss-mimic.css-data"),
//         require("postcss-tuesday.css-data")
//     ],
//     checkDuplications: true,
//     disableCheckCssVariables: true
// })

const env = process.env.NODE_ENV;
const isProd = env === "production";

const purgecss = require('@fullhuman/postcss-purgecss')({

    // Specify the paths to all of the template files in your project
    content: ['./src/*.rs'],

    keyframes: true,

    // Include any special characters you're using in this regular expression
    defaultExtractor: content => content.match(/[A-Za-z0-9-_:/]+/g) || []
})

module.exports = {
    plugins: [
        ...(isProd ? [purgecss] : []),
        require('postcss-import'),
        require('tailwindcss')('tailwind.config.js'),
        require('autoprefixer'),
        ...(isProd ? [ require('cssnano') ] : []),
        require('postcss-nested'),
    ]
}