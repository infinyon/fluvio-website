$(window).scroll(function () {
    handleScroll();
});

$(document).ready(function () {
    handleScroll();
});

handleScroll = () => {
    var swap = 96;
    var gradualTo = 167;
    var scrollTop = $(this).scrollTop();

    if (scrollTop > swap) {
        $('#navbar').addClass('fixed-top');
        $('#content').css('marginTop', '62px');
        $('.toc-area').css('top', '72px');
    } else if (scrollTop <= swap) {
        $('#navbar').removeClass('fixed-top');
        $('#content').css('marginTop', '0');
        $('.toc-area').css('marginTop', '10px');
        $('.side-toc .offset').css('marginTop', '0');
    }

    if (scrollTop >= swap && scrollTop < gradualTo) {
        $('.side-toc .offset').css('marginTop', (scrollTop - swap).toString() + 'px');
    } else if (scrollTop >= gradualTo) {
        $('.side-toc .offset').css('marginTop', '72px');
    }
}