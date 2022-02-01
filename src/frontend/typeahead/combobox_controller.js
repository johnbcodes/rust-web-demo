import { Controller } from "@hotwired/stimulus"
import Combobox from "@github/combobox-nav"

export default class extends Controller {
  static get targets() { return [ "input", "list" ] }

  listTargetConnected() {
    this.start()
  }

  start() {
    this.combobox?.destroy()
    this.combobox = new Combobox(this.inputTarget, this.listTarget)
    this.combobox.start()
  }

  stop() {
    this.combobox?.stop()
  }

  disconnect() {
    this.combobox?.destroy()
  }
}