export * from './EndpointSettings'
export * from './GlobalConfig'
export * from './Remote'

import { EndpointSettings } from './EndpointSettings'
import { Remote } from './Remote'
import { GlobalConfig } from './GlobalConfig'

export const accountProviders = { EndpointSettings, Remote, GlobalConfig }
