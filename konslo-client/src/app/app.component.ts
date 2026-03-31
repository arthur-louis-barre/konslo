import { Component } from '@angular/core';
import { RouterOutlet } from '@angular/router';

@Component({
  selector: 'app-root',
  imports: [RouterOutlet],
  templateUrl: './app.component.html',
  styleUrl: './app.component.css'
})
export class AppComponent {
  date = new Date().toLocaleDateString('en-US', { weekday: 'long', month: 'short', day: 'numeric' });
}
