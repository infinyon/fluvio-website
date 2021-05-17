$(document).ready(function () {
    $(".sidebar").mCustomScrollbar({
        theme: "minimal"
    });

    $('#dismiss, .page-overlay').on('click', function () {
        // hide sidebar
        $('.sidebar-area').removeClass('active');
        $('.sidebar').removeClass('ontop');
        $('.sidebar').css({ 'font-size': '' });
        $('.sidebar').css({ 'width': '' });

        // show elements
        $('.doc-area').css({ 'display': '' });
        $('.toc-area').css({ 'display': '' });
        $('.footer').css({ 'display': '' });

        // hide overlay
        $('.page-overlay').removeClass('active');
    });

    $('.sidebar-nav').on('click', function () {
        // open sidebar
        $('.sidebar-area').addClass('active');
        $('.sidebar').addClass('ontop');
        $('.sidebar').css({ 'font-size': '1.1em' });
        $('.sidebar').css({ 'width': '300px' });

        // fade in the overlay
        $('.page-overlay').addClass('active');
        $('.collapse.in').toggleClass('in');
        $('a[aria-expanded=true]').attr('aria-expanded', 'false');

        // hide elements
        $('.doc-area').css({ 'display': 'none' });
        $('.toc-area').css({ 'display': 'none' });
        $('.footer').css({ 'display': 'none' });
    });
});