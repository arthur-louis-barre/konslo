import { Component } from '@angular/core';
import {CalendarComponent} from "./calendar/calendar.component";

@Component({
  selector: 'k-right-sidebar',
    imports: [
        CalendarComponent
    ],
  templateUrl: './right-sidebar.component.html',
  styleUrl: './right-sidebar.component.css',
})
export class RightSidebarComponent {

}
