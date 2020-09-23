/**
 * @name 'Hugo Tags Filter'
 * @version 1.2.2
 * @license MIT  
 * @author PointyFar 
 * 
 * Modified extensively to cover "exclusive selection"
 * 
 */
class TagsFilter {

    /*
     * Initialize all parameters
     */
    constructor(config) {
        var defaultFilters = [
            {
                name: 'tag',
                prefix: 'tft-',
                buttonClass: 'tft-button',
                attrName: 'data-tags'
            }
        ]

        this.FILTERS = (config && config.filters) ? config.filters : defaultFilters;
        this.filterItemClass = (config && config.filterItemClass) ? config.filterItemClass : "tf-filter-item";
        this.showItemClass = (config && config.showItemClass) ? config.showItemClass : "tf-show";
        this.activeButtonClass = (config && config.activeButtonClass) ? config.activeButtonClass : "active";
        this.counterSelector = (config && config.counterSelector) ? config.counterSelector : "selectedItemCount";

        this.populateCount = (config && config.populateCount) ? config.populateCount : false;
        this.setDisabledButtonClass = (config && config.setDisabledButtonClass) ? config.setDisabledButtonClass : false;

        this.filterItems = document.getElementsByClassName(this.filterItemClass);
        this.selectedItemCount = this.filterItems.length;
        this.filterValues = {};

        for (var i = 0; i < this.FILTERS.length; i++) {
            this.FILTERS[i]['buttonTotal'] = document.getElementsByClassName(this.FILTERS[i]['buttonClass']).length;
            this.FILTERS[i]['filter'] = undefined;
            var fv = document.getElementsByClassName(this.FILTERS[i]['buttonClass']);

            this.filterValues[this.FILTERS[i]['name']] = [];
            for (var j = 0; j < fv.length; j++) {
                var v = fv[j].id.replace(this.FILTERS[i]["prefix"], '');
                this.filterValues[this.FILTERS[i]['name']][v] = { count: 0, selected: 0 };
            }
        }

        this.updateVisibleItems(true);
    }

    /*
     * checkFilter - Invoked by user through button click.
     */
    checkFilter(tag, tagType) {

        /* Single select - one or none (clicking again to remove selection) */
        for (var i = 0; i < this.FILTERS.length; i++) {
            if (this.FILTERS[i]['prefix'] === tagType) {
                if (this.FILTERS[i]['filter'] == tag) {
                    this.FILTERS[i]['filter'] = undefined
                } else {
                    this.FILTERS[i]['filter'] = tag
                }
                this.updateShowClass(this.FILTERS[i], tagType);
                this.updateVisibleItems(false);
            }
        }
    }

    /*
     * updateShowClass - Apply/Remove "show" class based on tag & tagType.
     */
    updateShowClass(filter) {
        var tagType = filter['prefix'];
        var tag = filter['filter'];
        var tagSelected = `${tagType}${tag}`;
        var filterButtons = document.getElementsByClassName(filter['buttonClass']);

        for (var j = 0; j < filterButtons.length; j++) {
            if (tagSelected === filterButtons[j].id) {
                this.addClassIfMissing(document.querySelector(`#${filterButtons[j].id}`), this.activeButtonClass);
            } else {
                this.delClassIfPresent(document.querySelector(`#${filterButtons[j].id}`), this.activeButtonClass);
            }
        }
    }

    updateVisibleItems(inInit) {
        this.selectedItemCount = 0;

        for (var i = 0; i < this.filterItems.length; i++) {
            // remove "show" class
            this.delClassIfPresent(this.filterItems[i], this.showItemClass);

            var visibility = 0;
            // show item only if visibility is true for all filters
            for (var j = 0; j < this.FILTERS.length; j++) {
                if (this.checkVisibility(this.FILTERS[j]['filter'], this.filterItems[i].getAttribute(this.FILTERS[j]['attrName']))) {
                    visibility++;
                }
            }

            // check if "show" class should be applied
            if (visibility === this.FILTERS.length) {
                if (!this.filterItems[i].classList.contains(this.showItemClass)) {
                    this.selectedItemCount++;
                    this.addClassIfMissing(this.filterItems[i], this.showItemClass);
                }
            }
        }

        this.updateFiltersCounts(inInit);
    }

    updateFiltersCounts(isInitial) {
        this.filterValues = this.updateFilterCount(this.filterValues, isInitial);
        this.populateCounters(this.filterValues);
        this.updateCounterSelector();
    }

    /*  
     * Update counts for all filters
     */
    updateFilterCount(fvc, isInitial) {
        if (isInitial) {
            for (var k in fvc) {
                for (var x = 0; x < this.filterItems.length; x++) {
                    var attrs = this.getAttrs(k, this.filterItems[x]);
                    for (var l = 0; l < attrs.length; l++) {
                        fvc[k][attrs[l]].count++;
                        fvc[k][attrs[l]].selected++;
                    }
                }
            }
        } else {
            var showing = document.getElementsByClassName(this.showItemClass);
            for (var k in fvc) {
                for (var k2 in fvc[k]) {
                    fvc[k][k2].selected = 0;
                }
            }
            for (var l = 0; l < showing.length; l++) {
                for (k in fvc) {
                    var attrs = this.getAttrs(k, showing[l]);
                    for (var m = 0; m < attrs.length; m++) {
                        fvc[k][attrs[m]].selected++;
                    }
                }
            }
        }

        return fvc;
    }


    populateCounters(fvc) {
        if (this.populateCount) {
            for (var i = 0; i < this.FILTERS.length; i++) {
                var fname = this.FILTERS[i]['name'];
                var cp = this.FILTERS[i]['countPrefix'];
                var sp = this.FILTERS[i]['selectedPrefix'];

                if (cp || sp) {
                    for (var k in fvc[fname]) {
                        if (cp) {
                            var cel = document.getElementById(`${cp}${k}`)
                            cel.textContent = fvc[fname][k].count;
                        }
                        if (sp) {
                            var sel = document.getElementById(`${sp}${k}`)
                            sel.textContent = fvc[fname][k].selected;
                            if (this.setDisabledButtonClass) {
                                if (sel.textContent == 0) {
                                    this.addClassIfMissing(document.getElementById(this.FILTERS[i]['prefix'] + k), this.setDisabledButtonClass);
                                } else {
                                    this.delClassIfPresent(document.getElementById(this.FILTERS[i]['prefix'] + k), this.setDisabledButtonClass)
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    /*
     * checkVisibility - Tests if attribute is included in list.
     */
    checkVisibility(filter, dataAttr) {
        if (filter) {
            var arr = dataAttr.split(" ")
                .filter(function (el) { return el.length > 0 });
            if (arr.indexOf(filter) >= 0) {
                return true
            }
        } else {
            return true
        }

    }

    /* 
     * getAttrs - returns an array of data-attr attributes of an element elem
     */
    getAttrs(attr, elem) {
        return elem.getAttribute('data-' + attr)
            .split(" ")
            .filter(function (el) {
                return el.length > 0
            });
    }

    addClassIfMissing(el, cn) {
        if (!el.classList.contains(cn)) {
            el.classList.add(cn);
        }
    }

    delClassIfPresent(el, cn) {
        if (el.classList.contains(cn)) {
            el.classList.remove(cn)
        }
    }

    updateCounterSelector() {
        if (document.getElementById(this.counterSelector)) {
            document.getElementById(this.counterSelector).textContent = `${this.selectedItemCount}`;
        }
    }
}

window['TagsFilter'] = TagsFilter;