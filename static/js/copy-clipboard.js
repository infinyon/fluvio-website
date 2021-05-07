function copyClipboard(id) {
    $("#" + id).click(function (event) {
        event.preventDefault();

        var link = [location.protocol, '//', location.host, location.pathname, "#", id].join('');
        navigator.clipboard.writeText(link);

        var copied = $("#" + id).children().last();
        copied.css("visibility", "visible");

        setTimeout(() => {
            copied.css("visibility", "hidden");
        }, 500);
    });
}