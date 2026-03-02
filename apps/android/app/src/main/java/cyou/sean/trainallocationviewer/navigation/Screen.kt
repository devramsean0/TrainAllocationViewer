package cyou.sean.trainallocationviewer.navigation

import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Search
import androidx.compose.material.icons.filled.Settings
import androidx.compose.ui.graphics.vector.ImageVector

sealed class Screen(
    val route: String,
    val label: String,
    val icon: ImageVector
) {
    data object Search : Screen(
        route = "search",
        label = "Search",
        icon = Icons.Default.Search
    )

    data object Settings : Screen(
        route = "settings",
        label = "Settings",
        icon = Icons.Default.Settings
    )

    companion object {
        val screens = listOf(Search, Settings)
    }
}

