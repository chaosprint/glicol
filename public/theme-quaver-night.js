// edited based on the tomorrow-night theme for ace editor

ace.define("ace/theme/tomorrow-night",["require","exports","module","ace/lib/dom"], function(acequire, exports, module) {

    exports.isDark = true;
    exports.cssClass = "ace-tomorrow-night";
    exports.cssText = `
    .ace-tomorrow-night .ace_gutter {
        background: #25282c;
        color: #C5C8C6
    }
      
      .ace-tomorrow-night .ace_print-margin {
        width: 1px;
        background: #25282c
      }
      
      .ace-tomorrow-night {
        background-color: #1D1F21;
        color: #C5C8C6
      }
      
      .ace-tomorrow-night .ace_cursor {
        color: #AEAFAD
      }
      
      .ace-tomorrow-night .ace_marker-layer .ace_selection {
        background: #373B41
      }
      
      .ace-tomorrow-night.ace_multiselect .ace_selection.ace_start {
        box-shadow: 0 0 3px 0px #1D1F21;
      }
      
      .ace-tomorrow-night .ace_marker-layer .ace_step {
        background: rgb(102, 82, 0)
      }
      
      .ace-tomorrow-night .ace_marker-layer .ace_bracket {
        margin: -1px 0 0 -1px;
        border: 1px solid #4B4E55
      }
      
      .ace-tomorrow-night .ace_marker-layer .ace_active-line {
        background: #282A2E
      }
      
      .ace-tomorrow-night .ace_gutter-active-line {
        background-color: #282A2E
      }
      
      .ace-tomorrow-night .ace_marker-layer .ace_selected-word {
        border: 1px solid #373B41
      }
      
      .ace-tomorrow-night .ace_invisible {
        color: #808080
      }
    .ace-tomorrow-night .ace_meta,
    .ace-tomorrow-night .ace_storage,
    .ace-tomorrow-night .ace_storage.ace_type,
    .ace-tomorrow-night .ace_support.ace_type {
    color: #a84275
    }
    .ace-tomorrow-night .ace_constant.ace_language,
    .ace-tomorrow-night .ace_keyword.ace_operator {
    color: #8959A8
    }
    .ace-tomorrow-night .ace_constant.ace_character,
    .ace-tomorrow-night .ace_constant.ace_numeric,
    .ace-tomorrow-night .ace_keyword.ace_other.ace_unit,
    .ace-tomorrow-night .ace_support.ace_constant,
    .ace-tomorrow-night .ace_variable.ace_parameter {
    color: #F5871F
    }
    .ace-tomorrow-night .ace_constant.ace_other {
    color: #666969
    }
    .ace-tomorrow-night .ace_invalid {
    color: #FFFFFF;
    background-color: #a84275
    }
    .ace-tomorrow-night .ace_invalid.ace_deprecated {
    color: #FFFFFF;
    background-color: #8959A8
    }
    .ace-tomorrow-night .ace_fold {
    background-color: #4271AE;
    border-color: #4D4D4C
    }
    .ace-tomorrow-night .ace_keyword,
    .ace-tomorrow-night .ace_entity.ace_name.ace_function,
    .ace-tomorrow-night .ace_support.ace_function,
    .ace-tomorrow-night .ace_variable {
    color: #4271AE
    }
    .ace-tomorrow-night .ace_support.ace_class,
    .ace-tomorrow-night .ace_support.ace_type {
    color: #C99E00
    }
    .ace-tomorrow-night .ace_heading,
    .ace-tomorrow-night .ace_markup.ace_heading,
    .ace-tomorrow-night .ace_string {
    color: #718C00
    }
    .ace-tomorrow-night .ace_entity.ace_name.ace_tag,
    .ace-tomorrow-night .ace_entity.ace_other.ace_attribute-name,
    .ace-tomorrow-night .ace_meta.ace_tag,
    .ace-tomorrow-night .ace_string.ace_regexp,
    .ace-tomorrow-night .ace_variable {
    color: #3E999F
    }
    .ace-tomorrow-night .ace_comment {
    color: #8E908C
    }
    .ace-tomorrow-night .ace_indent-guide {
    background: url(data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAACCAYAAACZgbYnAAAAE0lEQVQImWP4////f4bdu3f/BwAlfgctduB85QAAAABJRU5ErkJggg==) right repeat-y
    }
    `;
    
    var dom = acequire("../lib/dom");
    dom.importCssString(exports.cssText, exports.cssClass);
});