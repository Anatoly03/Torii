import { ToriiPlugin } from '@torii-project/core';

export class RecentProjectsPlugin extends ToriiPlugin {
    constructor() {
        super({
            id: 'core:recent-projects',
            version: '0.1.0',
        });
    }

    public override renderAnchor(anchorId: string) {
        switch (anchorId) {
            case 'welcome:sidebar:top':
                return import('./RecentProjectsView.vue');
            default:
                return;
        }
    }
}
