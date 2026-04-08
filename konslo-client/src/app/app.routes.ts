import { Routes } from '@angular/router';
import { HabitListComponent } from './habit/habit-list/habit-list.component';
import {HabitFormComponent} from "./habit/habit-form/habit-form.component";
import {authGuard} from "./auth/auth.guard";

export const routes: Routes = [
    { path: '', redirectTo: 'habits', pathMatch: 'full' },
    { path: 'habits', component: HabitListComponent, canActivate: [authGuard] },
    { path: 'new-habit', component: HabitFormComponent, canActivate: [authGuard]}
];
