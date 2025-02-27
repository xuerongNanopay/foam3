/**
 * @license
 * Copyright 2021 The FOAM Authors. All Rights Reserved.
 * http://www.apache.org/licenses/LICENSE-2.0
 */

foam.CLASS({
  package: 'foam.u2.table',
  name: 'UnstyledTableRow',
  extends: 'foam.u2.table.TableComponentView',

  mixins: ['foam.comics.v2.Clickable'],

  requires: [
    'foam.core.SimpleSlot',
    'foam.u2.CheckBox',
    'foam.u2.tag.Image',
    'foam.u2.view.OverlayActionListView',
    'foam.u2.table.UnstyledTableRowComponent'
  ],

  imports: [
    'click?',
    'config?',
    'dblclick?',
    'nestedPropsAndIndexes',
    'propertyNamesToQuery',
    'props',
    'stack?',
    'table'
  ],

  properties: [
    'obj',
    'projection',
    // Added for scrollTableView support
    {
      name: 'actionDAO',
      factory: function() { return this.data.data; }
    },
    'hoverSelection'
  ],

  methods: [
    function render() {
      this.SUPER();
      const obj = this.obj;
      var self = this;
      var nestedPropertyValues    = this.columnHandler.filterNestedPropertyValues(this.projection, this.nestedPropsAndIndexes[1]);
      var nestedPropertiesObjsMap = this.columnHandler.groupRelatedObjects(this.data.of, this.nestedPropsAndIndexes[0], nestedPropertyValues);
      this.addClass(this.data.myClass('tr')).
      on('mouseover', () => self.hoverSelection = obj).
      call(this.insertClick, [obj]). // TODO: why the bind(), call should apply to this anyway?
      enableClass(this.data.myClass('clickable'), ! this.config?.disableSelection).
      addClass(this.data.myClass('row')).
      style({ 'min-width': this.data.tableWidth_$ }).

      // If the multi-select feature is enabled, then we render a
      // Checkbox in the first cell of each row.
      callIf(this.data.multiSelectEnabled, function() {
        var slot = self.SimpleSlot.create();
        self
          .start('')
            .addClass(self.data.myClass('td'))
            .tag(self.CheckBox, { data: self.data.idsOfObjectsTheUserHasInteractedWith_[obj.id] ? !!self.data.selectedObjects[obj.id] : self.data.allCheckBoxesEnabled_ }, slot)
          .end()
          .enableClass(self.data.myClass('selected'), slot.value$.dot('data'));

        // Set up a listener so that when the user checks or unchecks
        // a box, we update the `selectedObjects` property.
        self.data.onDetach(slot.value$.dot('data').sub(function(_, __, ___, newValueSlot) {
          // If the user is checking or unchecking all boxes at once,
          // we only want to publish one propertyChange event, so we
          // trigger it from the listener in the table header instead
          // of here. This way we prevent a propertyChange being fired
          // for every single CheckBox's data changing.
          if ( self.data.togglingCheckBoxes_ ) return;

          // Remember that the user has interacted with this checkbox
          // directly. We need this because the ScrollTableView loads
          // tbody's in and out while the user scrolls, so we need to
          // handle the case when a user selects all, then unselects
          // a particular row, then scrolls far enough that the tbody
          // the selection was in unloads, then scrolls back into the
          // range where it reloads. We need to know if they've set
          // it to something already and we can't simply look at the
          // value on `selectedObjects` because then we won't know if
          // `selectedObjects[obj.id] === undefined` means they
          // haven't interacted with that checkbox or if it means they
          // explicitly set it to false. We could keep the key but set
          // the value to null, but that clutters up `selectedObjects`
          // because some values are objects and some are null. If we
          // use a separate set to remember which checkboxes the user
          // has interacted with, then we don't need to clutter up
          // `selectedObjects`.
          self.data.idsOfObjectsTheUserHasInteractedWith_[obj.id] = true;

          var checked = newValueSlot.get();

          if ( checked ) {
            var modification = {};
            self.data.data.find(obj.id).then(v => {
              modification[obj.id] = v;
              self.data.selectedObjects = Object.assign({}, self.data.selectedObjects, modification);
            });
          } else {
            var temp = Object.assign({}, self.data.selectedObjects);
            delete temp[obj.id];
            self.data.selectedObjects = temp;
          }
        }));
        // Store each CheckBox Element in a map so we have a reference
        // to them so we can set the `data` property of them when the
        // user checks the box to enable or disable all checkboxes.
        var checkbox = slot.get();
        self.data.checkboxes_[obj.id] = checkbox;
        checkbox.onDetach(function() {
          delete self.data.checkboxes_[obj.id];
        });
      });

      for ( var j = 0 ; j < this.data.columns_.length ; j++ ) {
        self.tag(self.UnstyledTableRowComponent, { data: self.data, col: this.data.columns_[j], nestedPropertiesObjsMap: nestedPropertiesObjsMap, obj: obj });
      }

      // Object actions
      var actions = this.data.getActionsForRow(obj);
      // When an action is taken, update the table
      obj?.sub('action', function() {
        if ( ! self.table ) return;
        self.table.updateValues = ! self.table.updateValues;
      });
      self
        .start('')
          .addClass(this.data.myClass('td'))
          .on('dblClick', e => {
            e.preventDefault();
            e.stopPropogation();
          })
          .attrs({ name: 'contextMenuCell' })
          .style({ flex: `0 0 ${this.data.EDIT_COLUMNS_BUTTON_CONTAINER_WIDTH}px` })
          .startContext({ stack: this.subStack })
          .tag(this.OverlayActionListView, {
            data: Object.values(actions),
            lazy: true,
            obj: obj,
            dao: self.actionDAO,
            showDropdownIcon: false,
            buttonStyle: 'TERTIARY',
            icon: 'images/Icon_More_Resting.svg'
          })
          .endContext()
        .end();
    }
  ]
});


foam.CLASS({
  package: 'foam.u2.table',
  name: 'UnstyledTableRowComponent',
  extends: 'foam.u2.table.TableComponentView',

  imports: [
    'colWidthUpdated',
    'props',
    'selectedColumnsWidth?'
  ],

  properties: [
    {
      name: 'colWidth',
      factory: function() {
        return this.selectedColumnsWidth && this.selectedColumnsWidth[this.propName] ?
        this.selectedColumnsWidth[this.propName] :
        this.columnHandler.returnPropertyForColumn(this.props, this.data.of, this.col, 'tableWidth');
      }
    },
    'col',
    'propName',
    'nestedPropertiesObjsMap',
    'obj'
  ],

  methods: [
    function render() {
      var self = this;
      this.propName = this.columnHandler.propertyNamesForColumnArray(this.col);
      [prop, objReturned] = this.getCellData(this.obj, this.col, this.nestedPropertiesObjsMap);

      // Added to maintain support for ScrollTableView that does not support resizable columns
      if ( this.colWidthUpdated$ && this.selectedColumnsWidth$ ) {
        this.onDetach(this.colWidthUpdated$.sub(function() {
          if ( self.selectedColumnsWidth[self.propName] )
            self.colWidth = self.selectedColumnsWidth[self.propName];
        }));
      }

      this
        .startContext({ controllerMode: 'VIEW' })
        .addClass(this.data.myClass('td'))
        .style({ flex: this.slot(function(colWidth) {
            return colWidth ? `1 0 ${colWidth}px` : `1 0 ${this.data.MIN_COLUMN_WIDTH_FALLBACK}px`;
          })
        })
        .call(function() {
          prop.tableCellFormatter.format(
            this,
            prop.f ? prop.f(objReturned) : null,
            objReturned,
            prop
          );
        })
        .endContext();
    }
  ]
});
