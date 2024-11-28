/**
 * @license
 * Copyright 2023 The FOAM Authors. All Rights Reserved.
 * http://www.apache.org/licenses/LICENSE-2.0
 */

foam.CLASS({
  package: 'foam.u2.wizard.wizardflow',
  name: 'AddFacadeWizardlet',
  extends: 'foam.u2.wizard.wizardflow.AddWizardlet',

  properties: [
    {
      class: 'Array',
      name: 'capabilityIds',
      description: 'Capabilities to facade'
    },
    {
      class: 'Map',
      name: 'facadeModelOverrides',
      description: 'Allows proving overrides for the facade model props'
    },
    {
      name: 'wizardletCls',
      value: 'foam.u2.wizard.wizardlet.AlternateFlowWizardlet'
    },
    {
      class: 'Map',
      name: 'wizardlets_'
    },
    {
      class: 'Map',
      name: 'factoryArgs',
      description: `Used to set up initial values of facade properties
      Expected format: {
        <capaId>: {<list of property: value pairs>}
      }`
    }
  ],
  methods: [
    function createPropertyName(capId) {
      return capId
      .split(/[.-]/)
      .map((word, index) => index == 0 ? word: word[0].toUpperCase() + word.slice(1))
      .join(''); 
    },
    function getWizardlet_(x) {
      let self = this;
      let facadeWizardlet = this.SUPER(x);

      // Set up the facade model using exisitng wizard properties
      // TODO: Dont remake this class if it has already been created
      var facadeModel = {
        package: 'FacadeWizardlet',
        name: this.wizardletId,
        properties: [
          ...this.capabilityIds.map(element => {
            let wi;
            if ( ! this.wizardlets_[element] ) {
              this.wizardlets_[element] = wi = x.wizardlets.find(w => w.id === element);
              if ( ! wi ) console.error('Cant find wizardlet with id', wi);
            } else {
              wi = this.wizardlets_[element];
            }
            // If the facade is set up to override view rebind the data so that the view uses correct wizardlet data
            if ( this.facadeModelOverrides[element]?.view ) {
              view = this.facadeModelOverrides[element].view;
              this.facadeModelOverrides[element].view = function(args, X) {
                args.data$ = wi.data$;
                return foam.u2.ViewSpec.createView(view, args, wi, X);
              }
            }
            return {
              name: this.createPropertyName(element),
              class: 'FObjectProperty',
              autoValidate: true,
              label: '',
              of: wi.of,
              view: (_, X) => {
                // This actually links the data of this property to the data of the wizardlet it is based of
                // This is really handy as it removes the need for extra LoaderInjectorSavers
                // Need to account for the case where the view is overriden
                return wi.sections[0].createView({}, { controllerMode: X.controllerMode$ });
              },
              ...(this.facadeModelOverrides[element] ?? {})
            }
          }),
          {
            class: 'Array',
            name: 'realWizardlets',
            hidden: true,
            factory: function() {
              return self.wizardlets_;
            },
            transient: true
          }
        ],
        methods: [
          function init() {
            Object.keys(self.wizardlets_).forEach(v => {
              // console.log(v, self.wizardlets_[v].getDataUpdateSub(), (self.wizardlets_[v].getDataUpdateSub()).$UID);
              this.onDetach(self.wizardlets_[v].getDataUpdateSub().sub(() => {
                this[self.createPropertyName(v)] = self.wizardlets_[v].data;
              }))
            });
          }
        ]
      };

      facadeClass = foam.core.Model.create(facadeModel).buildClass(x);
      foam.register(facadeClass);
      facadeWizardlet.of = facadeClass;

      var altAction = this.AlternateFlowAction.create({ alternateFlow: {
        class: 'foam.u2.wizard.AlternateFlow',
        name: 'goNext',
        label: 'Next',
        invisible: this.capabilityIds
      }});
      if ( altAction.canSkipData ) {
        altAction.isEnabled = function(isLoading_) { return !isLoading_ };
      }
      facadeWizardlet.dynamicActions.push(altAction);

      // Add a create loader in order to init the wizardlet data
      // TODO: Add loaders that can load data from the wizardlets directly
      facadeWizardlet.wao.loader = {
        class: 'foam.u2.wizard.data.CreateLoader',
        spec: { class: facadeClass.id }
      }
    }
  ]
});
