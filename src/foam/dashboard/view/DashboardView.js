/**
 * @license
 * Copyright 2022 The FOAM Authors. All Rights Reserved.
 * http://www.apache.org/licenses/LICENSE-2.0
 */

foam.CLASS({
  package: 'foam.dashboard.view',
  name: 'DashboardView',
  extends: 'foam.dashboard.view.Dashboard',
  mixins: ['foam.u2.layout.ContainerWidth'],

  imports: [
    'menuDAO',
    'displayWidth?',
    'document'
  ],

  requires: [
    'foam.nanos.menu.Menu'
  ],

  css: `
    ^ {
      height: 100%;
      width: 100%;
      display: flex;
      flex-direction: column;
    }
    ^main {
      height: fit-content;
      min-height: 600px;
      padding: 24px 32px;
      max-width: 160rem;
      margin: auto;
    }
    ^widget-container {
      width: 100%;
      display: grid;
      flex-grow: 1;
    }
  `,

  messages: [
    { name: 'TITLE', message: 'Dashboard' }
  ],

  properties: [
    {
      class: 'String',
      name: 'viewTitle',
      factory: function() {
        return this.TITLE;
      }
    },
    'dashboardTitle',
    {
      name: 'main',
      documentation: 'Should be set to true on the most top-level dashboard.',
      value: false
    },
    {
      name: 'width',
      documentation: 'The fixed number of grid columns for the dashboard.',
      expression: function(containerWidth) {
        if ( ! containerWidth ) return 'repeat(12, 1fr)';
        this.document.documentElement.style.setProperty('--dashboard-max-col', containerWidth.cols); 
        return `repeat(${containerWidth.cols}, 1fr)`;
      }
    },
    {
      name: 'height',
      documentation: 'The fixed number of grid rows for the dashboard.',
      value: 'min-content'
    },
    {
      name: 'gap',
      documentation: 'The px gap between dashboard widgets.',
      value: '1.6em'
    },
    {
      class: 'Map',
      name: 'widgets',
      documentation: 'Mapping of menu id to aspect ratio for widgets that will be displayed in the dashboard.',
      factory: function() {
        return {};
      },
      description: `The map of widgets used to render the dashboard. Allows specifying different columns for various
      widths of the dashboard container using '<displayWidth>Column'. A 0 column width implies that the number of columns should be equally split between 
      the widgets with 0 width, this provides an equivalent to css's grid-auto-column: 1fr behaviour.
      Ex:
        widgets: {
          <menu.id>: { column: 6, SMColumn: 12 .....}
        }

      Sample for equal width columns:
        widgets: {
          menuA: { column: 0 }
          menuB: { column: 0 }
          menuC: { column: 12 }
          menuD: { column: 0 }
          menuE: { column: 0 }
          menuF: { column: 0 }
        }
      Resulting grid template: 
        -menuA-- --menuB-
        menuC menuC menuC
        menuD menuE menuF
      In this case, menuA and menuB will split available columns between the two of them and menu D,E,F will split the columns in row three between them.
      `
    },
    {
      class: 'Map',
      name: 'containerMap'
    }
  ],

  methods: [
    function render() {
      this.SUPER();
      this.initContainerWidth();
      var widgetContainer = this.E()
        .addClass(this.myClass('widget-container'))
        .style({
          'grid-template-columns': this.width$,
          'grid-template-rows': this.height$,
          'grid-gap': this.gap$
        });

      Object.keys(this.widgets).map(async menuId => {
        let menu = await this.menuDAO.find(menuId);
        if ( menu ) {
          widgetContainer.startContext().start(menu.handler.view).style({
            'grid-column': this.containerMap$.map(v => {
              return v[menuId] ?? this.containerWidth?.cols;
            })
          }).end();
        } else {
          delete this.widgets[menuId];
        }
      });

      this.updateCols();
      this.containerWidth$.sub(this.updateCols);

      this
        .addClass(this.myClass())
        .enableClass(this.myClass('main'), this.main)
        .start()
          .hide(!this.dashboardTitle)
          .enableClass('h500', this.dashboardTitle)
          .style({ height: '2em' })
          .add(this.dashboardTitle)
        .end()
        .tag(widgetContainer)
    }
  ],

  listeners: [
    {
      name: 'updateCols',
      isFramed: true,
      code: function() {
        let cw = this.containerWidth;
        let currentWidgetSet = 0;
        let widgetSetCount = 0;
        let cm = {}
        Object.keys(this.widgets).forEach(v => {
          let colConfig = this.widgets[v];
          let col = colConfig[`${cw}Column`] ?? colConfig['column'];
          if ( col == 0 ) {
            if ( widgetSetCount == 0 )
              currentWidgetSet++;
            widgetSetCount++;
            cm[v] = `span calc(var(--dashboard-max-col)/var(--split-row-${currentWidgetSet}))`;
          } else {
            debugger;
            if ( widgetSetCount > 0 )
              this.document.documentElement.style.setProperty(`--split-row-${currentWidgetSet}`, widgetSetCount); 
            widgetSetCount = 0;
            cm[v] = 'span ' + col;
          }
        })
        this.containerMap = cm;
      }
    }
  ]
});

