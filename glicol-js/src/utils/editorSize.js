const setSize = () => {
    // console.log("set size")
    // if (window.state === "coding") {
      try {
        let w = document.getElementById('AppBar').offsetWidth
        let border =  document.documentElement.clientWidth - w
        let h = document.documentElement.clientHeight
        h = h - document.getElementById('AppBar').offsetHeight - border
        window.editor.container.style.width = `${w}px`
        window.editor.container.style.height = `${h}px`
        window.editor.resize()
      } catch (e) {}
    // } else {
      try {
        let w = window.innerWidth;
        let h = window.innerHeight;
        h -= document.getElementById('AppBar').offsetHeight
        setHeight(h)
        setWidth(w)
      } catch (e) {}
    // }
  }
window.onresize = setSize