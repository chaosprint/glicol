const posRegex = /(?<=pos\[)[^\]]+?(?=\])/g // /(?<=pos\[])[^\]]*(?=\])/g 
const lineRegex = /(?<=line\[)[^\]]+?(?=\])/g
const colRegex = /(?<=col\[)[^\]]+?(?=\])/g
const positivesRegex = /(?<=positives\[)[^\]]+?(?=\])/g
const negativesRegex = /(?<=negatives\[)[^\]]+?(?=\])/g
let pos = info.match(posRegex) ? parseInt(info.match(posRegex)[0]) : 0
let line = info.match(lineRegex) ? parseInt(info.match(lineRegex)[0]) : 0
let col = info.match(colRegex) ? parseInt(info.match(colRegex)[0]) : 0
// log(info.match(positivesRegex))
let positives = info.match(positivesRegex) ? info.match(positivesRegex)[0].replace("EOI", "END OF INPUT").split(",").join(" ||") : ""
let negatives = info.match(negativesRegex) ? info.match(negativesRegex)[0].split(",").join(" or") : ""
// log(pos, line, col, positives, negatives)
log(`%cError at line ${line}`, "background: #3b82f6; color:white; font-weight: bold")

let errline = window.code.split("\n")[line-1];
let styleErrLine = errline.slice(0, col-1) + "%c %c" + errline.slice(col-1);
log(styleErrLine, "font-weight: bold; background: #f472b6; color:white", "");

let positiveResult = positives.length > 0?
"expecting "+positives:""
log(
  `${"_".repeat(col-1 >=0?col-1:0)}%c^^^ ${positiveResult}${negatives.length > 0?"unexpected"+negatives:""}`,
    "font-weight: bold; background: #f472b6; color:white");