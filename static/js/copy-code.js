function addCopyButtons(clipboard) {
    const All = 0;
    const FirstLine = 1;
    const MultiLine = 2;

    document.querySelectorAll('.copy-code').forEach(function (copyCode) {
        var codeBlock = copyCode.nextElementSibling;
        var button = buildButton(codeBlock.innerHTML, All);
        codeBlock.parentNode.insertBefore(button, codeBlock);
    });

    document.querySelectorAll('.copy-first-line').forEach(function (copyCode) {
        var codeBlock = copyCode.nextElementSibling;
        var button = buildButton(codeBlock.innerHTML, FirstLine);
        codeBlock.parentNode.insertBefore(button, codeBlock);
    });

    document.querySelectorAll('.copy-multi-line').forEach(function (copyCode) {
        var codeBlock = copyCode.nextElementSibling;
        var button = buildButton(codeBlock.innerHTML, MultiLine);
        codeBlock.parentNode.insertBefore(button, codeBlock);
    });

    function copyContent(str, copy) {
        var result = "";

        if (copy == All) {
            result = str;
        } else {
            var currentIndex = 0;
            while ((index = str.indexOf('$', currentIndex)) > -1) {
                currentIndex = index + '$'.length;

                var endIndex = str.indexOf('\n', currentIndex);
                result += str.substr(currentIndex, endIndex - currentIndex + 1).trim() + '\n';
                currentIndex = index + endIndex;    

                if (copy == FirstLine) {
                    break;
                }
            }
        }
        
        return result;
    }

    function buildButton(html, copy) {
        var button = document.createElement('button');
        button.className = 'copy-code-button';
        button.type = 'button';
        button.innerText = 'Copy';

        button.addEventListener('click', function () {
            var div = document.createElement("div");
            div.innerHTML += html.replaceAll('class="hl">\n', 'class="hl">');
            var copyText = copyContent(div.innerText, copy);

            clipboard.writeText(copyText).then(function () {
                button.blur();
                button.innerText = 'Copied!';
                setTimeout(function () {
                    button.innerText = 'Copy';
                }, 2000);
            }, function (error) {
                button.innerText = 'Error';
            });
        });

        return button;
    }
}

if (navigator && navigator.clipboard) {
    addCopyButtons(navigator.clipboard);
}