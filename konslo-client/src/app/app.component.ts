import { Component } from '@angular/core';
import { RouterOutlet } from '@angular/router';
import {TopbarComponent} from "./shared/topbar/topbar.component";
import {LeftSidebarComponent} from "./shared/left-sidebar/left-sidebar.component";
import {RightSidebarComponent} from "./shared/right-sidebar/right-sidebar.component";

@Component({
  selector: 'app-root',
  imports: [RouterOutlet, TopbarComponent, LeftSidebarComponent, RightSidebarComponent],
  templateUrl: './app.component.html',
  styleUrl: './app.component.css'
})
export class AppComponent {

}
