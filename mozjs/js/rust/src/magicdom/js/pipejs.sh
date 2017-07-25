#!/bin/bash
# Script to separate the function and macro into separate lines and copy to Utilities.
# Intended for testing the generated selfhosted code.

cat DOMPointReadOnly.js | sed 's/#define/\'$'\n#define/g' | sed 's/function /\'$'\nfunction /g'  >> ../../../../src/builtin/Utilities.js
cat DOMPoint.js | sed 's/#define/\'$'\n#define/g' | sed 's/function /\'$'\nfunction /g' >> ../../../../src/builtin/Utilities.js
cat DOMQuad.js | sed 's/#define/\'$'\n#define/g' | sed 's/function /\'$'\nfunction /g' >> ../../../../src/builtin/Utilities.js
cat Node.js | sed 's/#define/\'$'\n#define/g' | sed 's/function /\'$'\nfunction /g' >> ../../../../src/builtin/Utilities.js
cat Attr.js | sed 's/#define/\'$'\n#define/g' | sed 's/function /\'$'\nfunction /g' >> ../../../../src/builtin/Utilities.js
cat Element.js | sed 's/#define/\'$'\n#define/g' | sed 's/function /\'$'\nfunction /g' >> ../../../../src/builtin/Utilities.js
cat HtmlElement.js | sed 's/#define/\'$'\n#define/g' | sed 's/function /\'$'\nfunction /g' >> ../../../../src/builtin/Utilities.js
