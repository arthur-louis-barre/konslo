import {Component, HostListener, inject, OnInit} from '@angular/core';
import {HabitsService} from './habits.service';
import {MatListModule} from '@angular/material/list';
import {GoalPeriod, HabitWithCheck} from './habits.model';
import {MatIconModule} from '@angular/material/icon';
import {MatButtonModule} from '@angular/material/button';
import {HabitCardComponent} from "./habit-card/habit-card.component";
import {Router} from "@angular/router";

@Component({
    selector: 'app-habits',
    imports: [MatListModule, MatIconModule, MatButtonModule, HabitCardComponent],
    templateUrl: './habits.component.html',
    styleUrl: './habits.component.css'
})
export class HabitsComponent implements OnInit {
    private router = inject(Router);
    private habitsService: HabitsService = inject(HabitsService);


    protected habits: HabitWithCheck[] = [];
    protected openDropdownId: number | null = null;

    @HostListener('document:click', ['$event'])
    close(event: MouseEvent) {
        if ((event.target as Element).closest('.h-card') == null) {
            this.openDropdownId = null;
        }
    }

    ngOnInit() {
        this.loadHabits();
    }

    loadHabits() {
        this.habitsService.getTodayHabits().subscribe((habits) => {
            this.habits = habits.map(h => {
                h.isCompleted = h.checks.length > 0 && (h.goal_value == h.checks[0].value);
                return h;
            })
        });
    }

    deleteHabit(id: number) {
        this.habitsService.deleteHabit(id).subscribe(() => {
            this.loadHabits();
        });
    }

    toggleDropdown(id: number) {
        if (this.openDropdownId === id)
            this.openDropdownId = null;
        else
            this.openDropdownId = id;
    }

    get habitsByPeriod() {
        const periods: GoalPeriod[] = ['day', 'week', 'month'];
        const labels: Record<GoalPeriod, string> = {
            day: 'Daily',
            week: 'Weekly',
            month: 'Monthly'
        }

        return periods
            .map(period => ({
                period,
                label: labels[period],
                habits: this.habits.filter(h => h.goal_period === period)
            }))
            .filter(hg => hg.habits.length > 0)
    }

    goToNewHabit() {
        this.router.navigate(['/new-habit']);
    }
 }