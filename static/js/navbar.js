$(window).scroll(function () {
    handleScroll();
});

$(document).ready(function () {
    handleScroll();
});

handleScroll = () => {
    var scrollTop = $(this).scrollTop();
    if (scrollTop > 65) {
        $('#navbar').addClass('fixed-top');
        $('#content').css('marginTop', '75px');
        $('.toc-area').css('top', '75px');
    } else if (scrollTop <= 65) {
        $('#navbar').removeClass('fixed-top');
        $('#content').css('marginTop', '0');
        $('.toc-area').css('marginTop', '10px');
        $('.side-toc .offset').css('marginTop', '0');
    }

    var gradualTo = 153;
    if (scrollTop >= 65 && scrollTop < gradualTo) {
        $('.side-toc .offset').css('marginTop', (scrollTop - 65).toString() + 'px');
    } else if (scrollTop >= gradualTo) {
        $('.side-toc .offset').css('marginTop', '87px');
    }
}