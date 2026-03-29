import { Component } from '@angular/core';
import { RouterOutlet } from '@angular/router';
import { MatToolbarModule } from '@angular/material/toolbar';
import {HabitListComponent} from "./habit/habit-list/habit-list.component";


@Component({
  selector: 'app-root',
  imports: [RouterOutlet, MatToolbarModule, HabitListComponent],
  templateUrl: './app.component.html',
  styleUrl: './app.component.css'
})
export class AppComponent {
  title = 'konslo-client';
  date = new Date().toLocaleDateString('en-US', { weekday: 'long', month: 'short', day: 'numeric' });
}
