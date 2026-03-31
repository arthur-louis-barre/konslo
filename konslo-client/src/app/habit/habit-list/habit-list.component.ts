import {Component, HostListener, inject, OnInit} from '@angular/core';
import {HabitService} from '../habit.service';
import {GoalPeriod, HabitWithCheck} from '../habit.model';
import {HabitCardComponent} from "./habit-card/habit-card.component";
import {Router} from "@angular/router";

@Component({
    selector: 'app-habits',
    imports: [HabitCardComponent],
    templateUrl: './habit-list.component.html',
    styleUrl: './habit-list.component.css'
})
export class HabitListComponent implements OnInit {
    private router = inject(Router);
    private habitsService: HabitService = inject(HabitService);


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
            this.habits = habits
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

    get greeting() {
        let hour = new Date().getHours();
        if (hour < 6) return "Good night"
        else if (hour < 12) return "Good morning"
        else if (hour < 18) return "Good afternoon"
        else return "Good evening"
    }

    goToNewHabit() {
        this.router.navigate(['/new-habit']);
    }
 }