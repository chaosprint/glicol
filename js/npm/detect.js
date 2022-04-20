export const detectOs = () => {
    var userAgent = window.navigator.userAgent,
        platform = window.navigator.platform,
        macosPlatforms = ['Macintosh', 'MacIntel', 'MacPPC', 'Mac68K'],
        windowsPlatforms = ['Win32', 'Win64', 'Windows', 'WinCE'],
        iosPlatforms = ['iPhone', 'iPad', 'iPod'],
        os = null;
    if (macosPlatforms.indexOf(platform) !== -1) {
        os = 'Mac OS';
    } else if (iosPlatforms.indexOf(platform) !== -1) {
        os = 'iOS';
    } else if (windowsPlatforms.indexOf(platform) !== -1) {
        os = 'Windows';
    } else if (/Android/.test(userAgent)) {
        os = 'Android';
    } else if (!os && /Linux/.test(platform)) {
        os = 'Linux';
    }
    return os;
}

export const detectBrowser = () => {
    const { userAgent } = navigator
    // alert(userAgent)
    // alert(detectOs());
    let name = "";
    let version = "0.0";
    if (userAgent.includes('Firefox/')) {
        // Firefox
        name = detectOs() === "Android" ? "Firefox for Android": "Firefox"
        version =  userAgent.split("Firefox/")[1]
    // } else if (userAgent.includes('Edg/')) {
        // name = "Edge"
    } else if (userAgent.includes('Chrome/')) {
        name = detectOs() === "Android" ? "Chrome for Android": "Chrome"
        version = userAgent.split("Chrome/")[1].split(" ")[0].split(".")[0]
    } else if (userAgent.includes('Safari/') && userAgent.includes('Version/') ) {
        name = detectOs() === "iOS" ? "Safari on iOS": "Safari"
        version = userAgent.split("Version/")[1].split(" ")[0]
    }
    return {
        name: name,
        version: parseFloat(version)
    }
}
