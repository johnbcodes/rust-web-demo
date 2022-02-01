import { Controller } from "@hotwired/stimulus"
import debounce from "lodash.debounce"

export default class extends Controller {
  static get targets() { return [ "submit" ] }

  initialize() {
    this.submit = debounce(this.submit.bind(this), 200)
  }

  connect() {
    this.submitTarget.hidden = true
  }

  submit() {
    this.submitTarget.click()
  }

  hideValidationMessage(event) {
    event.stopPropagation()
    event.preventDefault()
  }
}