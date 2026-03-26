import {Component, EventEmitter, inject, Input, Output} from '@angular/core';
import {HabitsService} from "../habits.service";
import {HabitWithCheck} from "../habits.model";
import {MatIcon} from "@angular/material/icon";

@Component({
    selector: 'app-habit-card',
    imports: [MatIcon],
    templateUrl: './habit-card.component.html',
    styleUrl: './habit-card.component.css',
})
export class HabitCardComponent {
    private habitsService = inject(HabitsService);

    protected isLoading = false;
    protected readonly Math = Math;

    @Input() habit!: HabitWithCheck;
    @Input() isDropdownOpen: boolean = false;
    @Output() habitUpdated = new EventEmitter<void>();
    @Output() habitDeleted = new EventEmitter<number>();
    @Output() dropdownToggled = new EventEmitter<number>();

    resetCheckValue(habit: HabitWithCheck) {
        if (this.isLoading) return;
        this.isLoading = true;

        if (habit.checks.length == 0) {
            this.isLoading = false;
        } else {
            this.habitsService
                .updateCheck(habit.id, habit.checks[0].id, {value: 0})
                .subscribe(() => {
                    this.habitUpdated.emit();
                    this.isLoading = false;
                });
        }


    }

    deleteHabit(habit: HabitWithCheck) {
        this.habitDeleted.emit(habit.id);
    }

    checkHabit(habit: HabitWithCheck) {
        if (this.isLoading) return;
        this.isLoading = true;

        if (habit.checks.length == 0) {
            this.habitsService
                .createCheck(habit.id, {value: 1, checked_at: new Date().toISOString()})
                .subscribe(() => {
                    this.habitUpdated.emit();
                    this.isLoading = false;
                });

        } else {
            this.habitsService
                .updateCheck(habit.id, habit.checks[0].id, {value: habit.checks[0].value + 1})
                .subscribe(() => {
                    this.habitUpdated.emit();
                    this.isLoading = false;
                });
        }
    }
}
