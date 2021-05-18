$(window).scroll(function () {
    handleScroll();
});

$(document).ready(function () {
    handleScroll();
});

handleScroll = () => {
    var topMax = 61;
    var scrollTop = $(this).scrollTop();

    if (scrollTop > topMax) {
        $('#navbar').addClass('fixed-top');
        $('#content').css('marginTop', topMax + 'px');
        $('.toc-area').css('top', '72px');
    } else if (scrollTop <= topMax) {
        $('#navbar').removeClass('fixed-top');
        $('#content').css('marginTop', '0');
        $('.toc-area').css('marginTop', '10px');
        $('.side-toc .offset').css('marginTop', '0');
    }

    var gradualTo = 135;
    if (scrollTop >= topMax && scrollTop < gradualTo) {
        $('.side-toc .offset').css('marginTop', (scrollTop - topMax).toString() + 'px');
    } else if (scrollTop >= gradualTo) {
        $('.side-toc .offset').css('marginTop', '72px');
    }
}