import {Component} from '@angular/core';

@Component({
    selector: 'k-topbar',
    templateUrl: './topbar.component.html',
    styleUrl: './topbar.component.css',
})
export class TopbarComponent {
    date = new Date().toLocaleDateString('en-US', {weekday: 'long', month: 'short', day: 'numeric'});
}
