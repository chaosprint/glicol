useEffect(() => {
    const completer = {
      getCompletions: function(editor, session, pos, prefix, callback) {
        var completions = comp

        /* You Can get to know how to add more cool 
        autocomplete features by seeing the ext-language-tools 
        file in the ace-buils folder */

        completions.forEach(i => {
          completions.push({
            caption: i.caption,
            snippet: i.snippet,
            type: i.type
          });
        });
        callback(null, completions);
      }
    };

    /* You can even use addCompleters instead of setCompleters like this :
       `addCompleter(completer)`;
     */

    setCompleters([completer]);
  }, [code]);
