import { Routes } from '@angular/router';
import { HabitListComponent } from './habit/habit-list/habit-list.component';
import {HabitFormComponent} from "./habit/habit-form/habit-form.component";

export const routes: Routes = [
    { path: '', redirectTo: 'habits', pathMatch: 'full' },
    { path: 'habits', component: HabitListComponent },
    { path: 'new-habit', component: HabitFormComponent}
];
