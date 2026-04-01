import {Component} from '@angular/core';
import {RouterLink, RouterLinkActive} from "@angular/router";

@Component({
    selector: 'k-left-sidebar',
    imports: [
        RouterLink,
        RouterLinkActive
    ],
    templateUrl: './left-sidebar.component.html',
    styleUrl: './left-sidebar.component.css',
})
export class LeftSidebarComponent {

}
