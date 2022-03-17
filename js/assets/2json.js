const fs = require('fs');
const folder = './';
var info = Object();
var source = "https://github.com/chaosprint/Dirt-Samples"
fs.readdir(folder, (_err, files) => {
    // info["selectedFromDirtSamples"] = files.filter(x=>x!=='.DS_Store')
    files.forEach(file => {
        if (file !== '.DS_Store' && file !== "2json.js") {
          info[file.replace(".wav", "")] = source
        }
    });
    let json = JSON.stringify(info)
    var outputFilename = '../src/sample-list.json';
    fs.writeFile(outputFilename, json, function(err) {
        if(err) {
          console.log(err);
        } else {
          console.log("JSON saved to " + outputFilename);
        }
    });
});
