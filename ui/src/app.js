// noinspection ES6UnusedImports
import * as Turbo from "@hotwired/turbo"
import { Application } from "@hotwired/stimulus"

import SearchController from "./typeahead/search_controller"
import ComboboxController from "./typeahead/combobox_controller";

window.Stimulus = Application.start()
Stimulus.register("typeahead--search", SearchController)
Stimulus.register("typeahead--combobox", ComboboxController)
