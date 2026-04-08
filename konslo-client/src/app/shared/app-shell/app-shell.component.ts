import { Component } from '@angular/core';
import {RouterOutlet} from "@angular/router";
import {LeftSidebarComponent} from "../left-sidebar/left-sidebar.component";
import {RightSidebarComponent} from "../right-sidebar/right-sidebar.component";
import {TopbarComponent} from "../topbar/topbar.component";

@Component({
  selector: 'k-app-shell',
  imports: [
    RouterOutlet,
    LeftSidebarComponent,
    RightSidebarComponent,
    TopbarComponent
  ],
  templateUrl: './app-shell.component.html',
  styleUrl: './app-shell.component.css',
})
export class AppShellComponent {

}
