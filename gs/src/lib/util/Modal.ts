import { displayModal } from '$lib/stores/state';
import { modalBody, modalTitle } from '$lib/stores/data';

export class Modal {
    private static instance: Modal;

    static getModal() {
        if (!Modal.instance) {
            Modal.instance = new Modal();
        }
        return Modal.instance;
    }

    public close() {
        displayModal.set(false);
    }

    public trigger(title: string, body: string) {
        modalTitle.set(title);
        modalBody.set(body);
        displayModal.set(true);
    }
}
