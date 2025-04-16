// This script generates an assets.json file that can be included in the webpage to load assets from the assets folder

const fs = require('fs');

function readAllAssets(path, assets = {}) {
    const files = fs.readdirSync(path);
    for (const file of files) {
        const filePath = path + "/" + file;
        if (fs.statSync(filePath).isDirectory()) {
            readAllAssets(filePath, assets);
        } else {
            assets[filePath.replace("assets/", "")] = fs.readFileSync(filePath).toJSON().data;
        }
    }
    return assets;
}

let assets = readAllAssets("assets");
fs.writeFileSync('./out/assets.json', JSON.stringify(assets));