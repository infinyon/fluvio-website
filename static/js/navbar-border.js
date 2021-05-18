$(window).scroll(function () {
    var scrollTop = $(this).scrollTop();

    if (scrollTop > 65) {
        $('.navbar-custom').addClass('bb');
    } else {
        $('.navbar-custom').removeClass('bb');
    }
});