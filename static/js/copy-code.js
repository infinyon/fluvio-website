function addCopyButtons(clipboard) {
    document.querySelectorAll('.copy-code').forEach(function (copyCode) {
        var codeBlock = copyCode.nextElementSibling;
        var button = buildButton(codeBlock.innerHTML, false);
        codeBlock.parentNode.insertBefore(button, codeBlock);
    });

    document.querySelectorAll('.copy-first-line').forEach(function (copyFirstLine) {
        var codeBlock = copyFirstLine.nextElementSibling;
        var button = buildButton(codeBlock.innerHTML, true);
        codeBlock.parentNode.insertBefore(button, codeBlock);
    });

    function buildButton(html, firstLine) {
        var button = document.createElement('button');
        button.className = 'copy-code-button';
        button.type = 'button';
        button.innerText = 'Copy';

        button.addEventListener('click', function () {
            var div = document.createElement("div");
            div.innerHTML += html.replaceAll('class="hl">\n', 'class="hl">');
            var copyText = "";

            if (firstLine) {
                copyText = div.innerText.substr(0, div.innerText.indexOf('\n'));
                copyText = copyText.indexOf('$') > -1 ? copyText.substr(copyText.indexOf('$') + 2) : copyText;
            } else {
                var copyText = div.innerText;
            }

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