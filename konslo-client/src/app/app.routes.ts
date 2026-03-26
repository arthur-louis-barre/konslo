import { Routes } from '@angular/router';
import { HabitsComponent } from './habits/habits.component';
import {HabitFormComponent} from "./habits/habit-form/habit-form.component";

export const routes: Routes = [
    { path: 'habits', component: HabitsComponent },
    { path: 'new-habit', component: HabitFormComponent}
];
