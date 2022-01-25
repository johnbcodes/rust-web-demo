// noinspection ES6UnusedImports
import * as Turbo from "@hotwired/turbo"
import { Application } from "@hotwired/stimulus"

import HelloController from "./hello_controller"

window.Stimulus = Application.start()
Stimulus.register("hello", HelloController)

console.log('Hello world!');