const regexp = /(?<=##)[^#]*(?=#)/g;  // this is working but not for nested
let match;
let toreplace = [];
while ((match = regexp.exec(code)) !== null) {
toreplace.push(match[0])
};

toreplace.map((str)=>{
    let result = str.includes('\n') || str.includes(';') ?
    Function(`'use strict'; return ()=>{${str}}`)()() : 
    Function(`'use strict'; return ()=>(${str})`)()();

    if (typeof result !== "undefined") {
        code = code.replace(`##${str}#`, result)
    } else {
        code = code.replace(`##${str}#`, "")
    };
});
code