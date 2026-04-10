import {Component, inject, input, output} from '@angular/core';
import {HabitService} from "../../habit.service";
import {HabitWithCheck} from "../../habit.model";

@Component({
    selector: 'k-habit-card',
    templateUrl: './habit-card.component.html',
    styleUrl: './habit-card.component.css',
})
export class HabitCardComponent {
    private habitsService = inject(HabitService);

    protected isLoading = false;

    selectedDate = input<string>();
    habit = input.required<HabitWithCheck>();
    isDropdownOpen = input<boolean>(false);

    habitUpdated = output<void>();
    habitDeleted = output<string>();
    dropdownToggled = output<string>();

    // CLICK-HANDLERS

    addCheck() {
        if (this.isLoading) return;
        this.isLoading = true;

        this.habitsService
            .addCheck(this.habit().id, 1, this.selectedDate())
            .subscribe({
                next: () => {
                    this.habitUpdated.emit();
                    this.isLoading = false
                },
                error: (err) => {
                    console.error(err);
                    this.isLoading = false;
                }
            });
    }

    resetCheck() {
        this.habitsService
            .resetCheck(this.habit().id, this.selectedDate())
            .subscribe({
                next: () => { this.habitUpdated.emit(); },
                error: (err) => { console.error(err); }
            })
    }

    deleteHabit() {
        this.habitDeleted.emit(this.habit().id);
    }

    toggleDropdown() {
        this.dropdownToggled.emit(this.habit().id)
    }

    // GETTERS

    get isCompleted() {
        return this.progressPercent >= 100;
    }

    get progressPercent() {
        const progressValue = this.progressValue;
        const goalValue = this.habit().goal_value;

        return Math.round(progressValue / goalValue * 100)
    }

    get progressValue() {
        return this.habit().checks.reduce((sum, check) => sum + check.value, 0);
    }
}
