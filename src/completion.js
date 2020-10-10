export default "sin|saw|squ|imp|lpf|hpf|noiz|sampler|seq|loop|speed|choose|mul|add|envperc|linrange".split("|").map(
    i => {
        return {
            caption: i,
            snippet: i,
            type: i
        }
    }
)

  /* You Can get to know how to add more cool 
  autocomplete features by seeing the ext-language-tools 
  file in the ace-buils folder */